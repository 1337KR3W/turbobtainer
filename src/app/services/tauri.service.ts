import { Injectable, signal } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { AppState } from '../models/download-state.model';

@Injectable({
  providedIn: 'root'
})
export class TauriService {
  // 1. Inicializamos el estado en 'IDLE' (Reposo)
  // Usamos un Signal para que la UI se entere de los cambios automáticamente
  state = signal<AppState>({
    status: 'IDLE'
  });

  constructor() { }

  /**
   * Cambia el estado a ANALYZING y llama a Rust.
   * Si tiene éxito pasa a READY, si falla a ERROR.
   */
  async obtenerMetadata(url: string, tipo: 'audio' | 'video') {
    // Actualizamos estado a ANALYZING
    this.state.set({ status: 'ANALYZING', tipoSeleccionado: tipo });

    try {
      // Llamada al comando de Rust (PMO-6)
      const titulo = await invoke<string>('check_video_url', { url });

      // Éxito: Pasamos a READY con el título
      this.state.set({
        status: 'READY',
        videoTitle: titulo,
        tipoSeleccionado: tipo
      });
    } catch (error) {
      // Fallo: Pasamos a ERROR con el mensaje legible
      this.state.set({
        status: 'ERROR',
        mensaje: error as string
      });
    }
  }

  /**
   * Limpia el estado para volver al buscador inicial
   */
  reset() {
    this.state.set({ status: 'IDLE' });
  }
}