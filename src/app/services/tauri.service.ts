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
  private readonly injector = inject(Injector);

  constructor() {
    this.setupListeners();
  }

  private async setupListeners() {
    try {
      this.unlistenProgress = await listen<number>('download-progress', (event) => {
        const progress = event.payload;

        if (progress === 1) {
          this._state.update(s => ({
            ...s,
            status: 'SUCCESS',
            progress: 1
          }));
        } else {
          this._state.update(s => ({
            ...s,
            status: 'DOWNLOADING',
            progress: progress
          }));
        }
      });
    } catch (error) {
      console.error('Error:', error);
    }
  }
  async getMetadata(url: string, type: 'audio' | 'video') {
    this.urlMemoria = url;
    this._state.set({ status: 'ANALYZING', selectedType: type });

    try {
      const metadata = await invoke<any>('check_video_url', { url });
      this._state.set({
        status: 'READY',
        selectedType: type,
        videoTitle: metadata.title,
        thumbnail: metadata.thumbnail,
        duration: metadata.duration,
        size: metadata.size,
        progress: 0
      });
    } catch (error) {
      this._state.set({
        status: 'ERROR',
        message: error as string
      });
    }
  }
  async getMetadataGallery(url: string) {
    this.urlMemoria = url;
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
    const actual = this._state();
    if (actual.status !== 'READY') return;

    this._state.update(s => ({ ...s, status: 'DOWNLOADING', progress: 0 }));

    try {
      if (actual.selectedType === 'gallery') {
        const totalItems = actual.imageCount || 0;
        const resultado = await invoke<string>('download_gallery', {
          url: this.urlMemoria,
          totalItems: totalItems
        });

        console.log(resultado);
        this._state.update(s => ({
          ...s,
          status: 'SUCCESS',
          progress: 1
        }));

      } else {
        await invoke('download_video', {
          url: this.urlMemoria,
          stype: actual.selectedType
        });
      }

    } catch (error) {
      console.error('Error en la descarga:', error);
      this._state.set({ status: 'ERROR', message: error as string });
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