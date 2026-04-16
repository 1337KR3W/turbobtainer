import { Component, Input, Output, EventEmitter, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { IonButton, IonCard, IonCardContent, IonCardHeader, IonCardSubtitle, IonCardTitle, IonCol, IonIcon, IonInput, IonItem, IonProgressBar, IonRow, IonSpinner, IonLabel, IonGrid, IonList, IonThumbnail, IonFooter } from '@ionic/angular/standalone';
import { UtilsService } from '../../services/utils.service';
import { SupportGrid } from '../support-grid/support-grid';
import { TauriService } from '../../services/tauri.service';

@Component({
  selector: 'app-download-manager',
  standalone: true,
  imports: [CommonModule, FormsModule, IonCard, IonCardHeader, IonCardTitle, IonCardContent,
    IonItem, IonInput, IonButton, IonIcon, IonRow, IonCol,
    IonSpinner, IonProgressBar, IonCardSubtitle, IonLabel, IonGrid, IonList, IonThumbnail, IonFooter],
  templateUrl: './download-manager.component.html',
  styleUrls: ['./download-manager.component.scss']
})
export class DownloadManagerComponent {

  private readonly utils = inject(UtilsService);
  public readonly tauri = inject(TauriService);

  public readonly supportedPlatforms = this.utils.MASTER_SITES;

  @Input() url: string = '';
  @Output() urlChange = new EventEmitter<string>();
  @Output() analyze = new EventEmitter<'audio' | 'video' | 'gallery'>();
  @Output() download = new EventEmitter<void>();
  @Output() cancelDld = new EventEmitter<void>();

  constructor() {

    this.utils.initializeIcons();
    this.utils.setRandomAscii();
  }

  get status() {
    return this.tauri.state().status;
  }

  isVideoUrl(): boolean {
    return this.utils.isVideoUrl(this.url);
  }

  isGalleryUrl(): boolean {
    return this.utils.isGalleryUrl(this.url);
  }
  async openSupportModal() {
    const modal = await this.utils.modalCtrl.create({
      component: SupportGrid,
    });
    return await modal.present();
  }
  setRandomBackGround() {
    this.utils.setRandomAscii();
  }

}