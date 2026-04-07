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
      const metadata = await invoke<any>('check_video_url', { url });
      this._state.set({
        status: 'READY',
        tipoSeleccionado: tipo,
        videoTitle: metadata.title,
        thumbnail: metadata.thumbnail,
        duration: metadata.duration,
        size: metadata.size,
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