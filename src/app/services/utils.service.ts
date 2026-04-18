import { inject, Injectable, signal } from '@angular/core';
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
    imageOutline,
    checkmarkCircle,
    tvOutline,
    arrowBackOutline,
    openOutline
} from 'ionicons/icons';
import { TauriService } from './tauri.service';
import { ModalController } from '@ionic/angular/standalone';
import { SUPPORTED_PLATFORMS } from '../models/supported-platforms.model';
import { ASCII_DESIGNS } from '../models/background-ascii-art';

@Injectable({
    providedIn: 'root'
})
export class UtilsService {
    public tauriService = inject(TauriService);

    public readonly modalCtrl = inject(ModalController);

    public readonly MASTER_SITES = SUPPORTED_PLATFORMS;
    public currentAscii = signal<string>(ASCII_DESIGNS[0]);
    constructor() {
        this.setRandomAscii();
    }
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
            'checkmark-circle': checkmarkCircle,
            'tv-outline': tvOutline,
            'arrow-back-outline': arrowBackOutline,
            'open-outline': openOutline
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
        return this.MASTER_SITES.some(sitio => urlLower.includes(sitio));
    }
    public getPlatformLogo(url: string): string {
        if (!url) return '';
        const urlLower = url.toLowerCase();
        const platform = this.MASTER_SITES.find(site => urlLower.includes(site));
        if (platform) {
            return this.getIconUrl(platform);
        }

        if (urlLower.includes('youtube.com') || urlLower.includes('youtu.be')) {
            return this.getIconUrl('youtube');
        }

        try {
            const domain = new URL(url).hostname.replace('www.', '');
            return this.getIconUrl(domain);
        } catch {
            return '';
        }
    }

    public setRandomAscii() {
        const randomIndex = Math.floor(Math.random() * ASCII_DESIGNS.length);
        this.currentAscii.set(ASCII_DESIGNS[randomIndex]);
    }
}