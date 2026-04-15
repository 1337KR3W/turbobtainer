import { inject, Injectable } from '@angular/core';
import { addIcons } from 'ionicons';
import {
    close,
    searchOutline,
    globeOutline,
    downloadOutline,
    trashOutline,
    alertCircleOutline,
    musicalNotesOutline,
    videocamOutline,
    checkmarkDoneOutline,
    cloudDownloadOutline,
    logoYoutube,
    imageOutline
} from 'ionicons/icons';
import { TauriService } from './tauri.service';
import { ModalController } from '@ionic/angular/standalone';
import { SUPPORTED_PLATFORMS } from '../models/supported-platforms.model';

@Injectable({
    providedIn: 'root'
})
export class UtilsService {
    public tauriService = inject(TauriService);

    public readonly modalCtrl = inject(ModalController);

    public readonly MASTER_SITES = SUPPORTED_PLATFORMS;

    public initializeIcons() {
        addIcons({
            'close': close,
            'search-outline': searchOutline,
            'globe-outline': globeOutline,
            'download-outline': downloadOutline,
            'trash-outline': trashOutline,
            'alert-circle-outline': alertCircleOutline,
            'musical-notes-outline': musicalNotesOutline,
            'videocam-outline': videocamOutline,
            'checkmark-done-outline': checkmarkDoneOutline,
            'cloudDownloadOutline': cloudDownloadOutline,
            'logoYoutube': logoYoutube,
            'image-outline': imageOutline,
        });
    }

    public getIconUrl(sitio: string): string {
        const domain = sitio.includes('.') ? sitio : `${sitio}.com`;
        return `https://www.google.com/s2/favicons?domain=${domain}&sz=64`;
    }

    public isVideoUrl(url: string): boolean {
        if (!url) return false;
        const urlLower = url.toLowerCase();
        return urlLower.includes('youtube.com') || urlLower.includes('youtu.be');
    }

    public isGalleryUrl(url: string): boolean {
        if (!url) return false;
        const urlLower = url.toLowerCase();
        // Comprobamos contra la lista de plataformas del modelo de datos
        return this.MASTER_SITES.some(sitio => urlLower.includes(sitio));
    }
}