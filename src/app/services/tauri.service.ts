import { Injectable, signal, OnDestroy, inject, Injector } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { AppState } from '../models/download-state.model';
import { UtilsService } from './utils.service';

@Injectable({
  providedIn: 'root'
})
export class TauriService implements OnDestroy {
  private readonly _state = signal<AppState>({ status: 'IDLE' });
  public state = this._state.asReadonly();
  private unlistenProgress?: UnlistenFn;
  private urlMemoria: string = '';
  private injector = inject(Injector);

  constructor() {

    this.setupListeners();
    this.testGallery();
  }

  async testGallery() {
    try {
      const version = await invoke('check_gallery_binary');
      console.log('Gallery-dl Version:', version);
    } catch (error) {
      console.error('Error calling sidecar:', error);
    }
  }

  private async setupListeners() {
    try {
      this.unlistenProgress = await listen<number>('download-progress', (event) => {
        const progreso = event.payload;

        if (progreso === 1) {
          this._state.update(s => ({
            ...s,
            status: 'SUCCESS',
            progreso: 1
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
      console.error('Error:', error);
    }
  }
  async getMetadata(url: string, tipo: 'audio' | 'video') {
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
  async getMetadataGallery(url: string) {
    this.urlMemoria = url;
    const utils = this.injector.get(UtilsService);
    const logo = utils.getPlatformLogo(url);

    // Reset inicial para evitar que se queden thumbnails de búsquedas anteriores
    this._state.set({
      status: 'ANALYZING',
      tipoSeleccionado: 'gallery',
      thumbnail: undefined // <--- Muy importante
    });

    try {
      const metadata = await invoke<any>('check_gallery_url', { url });

      this._state.set({
        status: 'READY',
        tipoSeleccionado: 'gallery',
        videoTitle: metadata.title,
        sourceLogo: logo, // <--- Esto activará el logo en el HTML
        imageCount: metadata.count,
        mensaje: metadata.description,
        progreso: 0
      });
    } catch (error) {
      this._state.set({
        status: 'ERROR',
        mensaje: error as string
      });
    }
  }

  async checkUrlType(url: string, tipo: 'audio' | 'video' | 'gallery') {
    if (tipo === 'gallery') {
      return this.getMetadataGallery(url);
    } else {
      return this.getMetadata(url, tipo);
    }
  }

  async startDownload() {
    const actual = this._state();
    if (actual.status !== 'READY') return;

    this._state.update(s => ({ ...s, status: 'DOWNLOADING', progreso: 0 }));

    try {
      if (actual.tipoSeleccionado === 'gallery') {
        const resultado = await invoke<string>('download_gallery', {
          url: this.urlMemoria
        });

        console.log(resultado);
        this._state.update(s => ({
          ...s,
          status: 'SUCCESS',
          progreso: 1
        }));

      } else {
        await invoke('download_video', {
          url: this.urlMemoria,
          tipo: actual.tipoSeleccionado
        });
      }

    } catch (error) {
      console.error('Error en la descarga:', error);
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