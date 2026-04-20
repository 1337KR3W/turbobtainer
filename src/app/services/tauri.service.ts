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
  private urlMemory: string = ''; // Renombrado de urlMemoria
  private readonly injector = inject(Injector);

  constructor() {
    this.setupListeners();
  }

  private async setupListeners() {
    try {
      this.unlistenProgress = await listen<number>('download-progress', (event) => {
        const progress = event.payload;
        this._state.update(s => ({
          ...s,
          status: progress === 1 ? 'SUCCESS' : 'DOWNLOADING',
          progress: progress
        }));
      });
    } catch (error) {
      console.error('Listener setup error:', error);
    }
  }

  async getMetadata(url: string, type: 'audio' | 'video') {
    this.urlMemory = url;
    this._state.set({ status: 'ANALYZING', selectedType: type });

    try {
      // Rust ahora devuelve VideoMetadata con has_playlist
      const metadata = await invoke<any>('check_video_url', { url });

      this._state.set({
        status: 'READY',
        selectedType: type,
        videoTitle: metadata.title,
        thumbnail: metadata.thumbnail,
        duration: metadata.duration,
        size: metadata.size,
        hasPlaylist: metadata.has_playlist, // <--- Nueva propiedad
        shouldDownloadPlaylist: false,      // Por defecto no descargamos la lista completa
        progress: 0,
      });
    } catch (error) {
      this._state.set({
        status: 'ERROR',
        message: error as string
      });
    }
  }

  togglePlaylist(value: boolean) {
    this._state.update(s => ({ ...s, shouldDownloadPlaylist: value }));
  }

  async getMetadataGallery(url: string) {
    this.urlMemory = url;
    const utils = this.injector.get(UtilsService);
    const logo = utils.getPlatformLogo(url);

    this._state.set({
      status: 'ANALYZING',
      selectedType: 'gallery',
      thumbnail: undefined
    });

    try {
      const metadata = await invoke<any>('check_gallery_url', { url });

      this._state.set({
        status: 'READY',
        selectedType: 'gallery',
        videoTitle: metadata.title,
        sourceLogo: logo,
        imageCount: metadata.count,
        message: metadata.description,
        progress: 0
      });
    } catch (error) {
      this._state.set({
        status: 'ERROR',
        message: error as string
      });
    }
  }

  async checkUrlType(url: string, type: 'audio' | 'video' | 'gallery') {
    if (type === 'gallery') {
      return this.getMetadataGallery(url);
    } else {
      return this.getMetadata(url, type);
    }
  }

  async startDownload() {
    const current = this._state(); // Renombrado de actual
    if (current.status !== 'READY') return;

    this._state.update(s => ({ ...s, status: 'DOWNLOADING', progress: 0 }));

    try {
      if (current.selectedType === 'gallery') {
        const totalItems = current.imageCount || 0;
        await invoke<string>('download_gallery', {
          url: this.urlMemory,
          totalItems: totalItems
        });

        this._state.update(s => ({ ...s, status: 'SUCCESS', progress: 1 }));
      } else {
        // --- AQUÍ EL CAMBIO CLAVE PARA RUST ---
        await invoke('download_video', {
          url: this.urlMemory,
          stype: current.selectedType,
          downloadPlaylist: current.shouldDownloadPlaylist || false // Enviamos el booleano
        });
      }
    } catch (error) {
      console.error('Download error:', error);
      this._state.set({ status: 'ERROR', message: error as string });
    }
  }

  reset() {
    this.urlMemory = '';
    this._state.set({ status: 'IDLE' });
  }

  ngOnDestroy() {
    if (this.unlistenProgress) {
      this.unlistenProgress();
    }
  }
}