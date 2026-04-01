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
      // UNIFICADO: Un solo listener con lógica de finalización
      this.unlistenProgress = await listen<number>('download-progress', (event) => {
        const progreso = event.payload;

        if (progreso >= 1.0) {
          this._state.update(s => ({
            ...s,
            status: 'SUCCESS',
            progreso: 1.0
          }));
        } else {
          this._state.update(s => ({
            ...s,
            status: 'DOWNLOADING',
            progreso: progreso
          }));
        }
      });
    } catch (error) {
      console.error('Error al suscribirse a eventos de progreso:', error);
    }
  }

  async obtenerMetadata(url: string, tipo: 'audio' | 'video') {
    this.urlMemoria = url;
    this._state.set({ status: 'ANALYZING', tipoSeleccionado: tipo });

    try {
      const titulo = await invoke<string>('check_video_url', { url });
      this._state.set({
        status: 'READY',
        videoTitle: titulo,
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