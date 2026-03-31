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

  // 1. VARIABLE NUEVA: Guardamos la URL para usarla en el momento de la descarga
  private urlMemoria: string = '';

  constructor() {
    this.setupListeners();
  }

  private async setupListeners() {
    try {
      this.unlistenProgress = await listen<number>('download-progress', (event) => {
        // Actualizamos el progreso con el valor que viene de Rust
        this._state.update(s => ({
          ...s,
          status: 'DOWNLOADING',
          progreso: event.payload
        }));
      });
    } catch (error) {
      console.error('Error al suscribirse a eventos de progreso:', error);
    }
  }

  async obtenerMetadata(url: string, tipo: 'audio' | 'video') {
    // 2. GUARDAMOS LA URL: Antes de nada, la ponemos en memoria
    this.urlMemoria = url;

    this._state.set({ status: 'ANALYZING', tipoSeleccionado: tipo });

    try {
      const titulo = await invoke<string>('check_video_url', { url });
      this._state.set({
        status: 'READY',
        videoTitle: titulo,
        tipoSeleccionado: tipo
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
      // 3. CAMBIO CRÍTICO: Enviamos 'this.urlMemoria' en lugar de 'actual.videoTitle'
      await invoke('download_video', {
        url: this.urlMemoria,
        tipo: actual.tipoSeleccionado
      });
    } catch (error) {
      this._state.set({ status: 'ERROR', mensaje: error as string });
    }
  }

  reset() {
    this.urlMemoria = ''; // Limpiamos la memoria al resetear
    this._state.set({ status: 'IDLE' });
  }

  ngOnDestroy() {
    if (this.unlistenProgress) {
      this.unlistenProgress();
    }
  }
}