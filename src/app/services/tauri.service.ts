import { Injectable, signal, OnDestroy } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { AppState } from '../models/download-state.model';

@Injectable({
  providedIn: 'root'
})
export class TauriService implements OnDestroy {
  private readonly _state = signal<AppState>({ status: 'IDLE' });
  public state = this._state.asReadonly();
  private unlistenProgress?: UnlistenFn;
  private urlMemoria: string = '';

  private formatBytes(bytes: any): string {
    // Si es null, undefined o la cadena "null" que devuelve to_string() en Rust
    if (!bytes || bytes === 'null' || bytes === 'NA') return 'Size: NA';

    // Quitamos cualquier cosa que no sea número (por si acaso)
    const numericSize = String(bytes).replace(/[^0-9]/g, '');
    const b = parseInt(numericSize);

    if (isNaN(b) || b <= 0) return 'Size: NA';

    return (b / (1024 * 1024)).toFixed(2) + ' MB';
  }
  constructor() {
    this.setupListeners();
  }

  private async setupListeners() {
    try {
      this.unlistenProgress = await listen<number>('download-progress', (event) => {
        const progreso = event.payload;

        // Solo pasamos a SUCCESS si es exactamente 1.0
        if (progreso === 1) {
          this._state.update(s => ({
            ...s,
            status: 'SUCCESS',
            progreso: 1
          }));
        } else {
          // Cualquier otro valor (incluyendo 0.99) mantiene el estado DOWNLOADING
          this._state.update(s => ({
            ...s,
            status: 'DOWNLOADING',
            progreso: progreso
          }));
        }
      });
    } catch (error) {
      console.error('Error:', error);
    }
  }
  async obtenerMetadata(url: string, tipo: 'audio' | 'video') {
    this.urlMemoria = url;
    this._state.set({ status: 'ANALYZING', tipoSeleccionado: tipo });

    try {

      const data = await invoke<{ title: string, thumbnail: string, duration: string, size: string }>('check_video_url', { url });
      console.log('Datos recibidos de Rust:', data); // <-- Añade este log para depurar
      this._state.set({
        status: 'READY',
        videoTitle: data.title,
        thumbnailUrl: data.thumbnail,
        duration: data.duration,
        size: this.formatBytes(data.size),
        tipoSeleccionado: tipo,
        progreso: 0
      });
    } catch (error) {
      this._state.set({
        status: 'ERROR',
        mensaje: error as string
      });
    }
  }

  async iniciarDescarga() {
    const actual = this._state();
    if (actual.status !== 'READY') return;

    this._state.update(s => ({ ...s, status: 'DOWNLOADING', progreso: 0 }));

    try {
      await invoke('download_video', {
        url: this.urlMemoria,
        tipo: actual.tipoSeleccionado
      });
    } catch (error) {
      this._state.set({ status: 'ERROR', mensaje: error as string });
    }
  }

  reset() {
    this.urlMemoria = '';
    this._state.set({ status: 'IDLE' });
  }

  ngOnDestroy() {
    if (this.unlistenProgress) {
      this.unlistenProgress();
    }
  }
}