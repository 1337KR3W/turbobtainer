import { Component, Input, Output, EventEmitter } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { IonButton, IonCard, IonCardContent, IonCardHeader, IonCardSubtitle, IonCardTitle, IonCol, IonIcon, IonInput, IonItem, IonProgressBar, IonRow, IonSpinner, IonLabel, IonGrid, IonList, IonThumbnail, IonFooter } from '@ionic/angular/standalone';
import { UtilsService } from '../../services/utils.service';
import { SupportGrid } from '../support-grid/support-grid';

@Component({
  selector: 'app-download-manager',
  standalone: true,
  imports: [CommonModule, FormsModule, IonCard, IonCardHeader, IonCardTitle, IonCardContent,
    IonItem, IonInput, IonButton, IonIcon, IonRow, IonCol,
    IonSpinner, IonProgressBar, IonCardSubtitle, IonLabel, IonGrid, IonList, IonThumbnail, IonFooter],
  templateUrl: './download-manager.component.html',
  styleUrls: ['./download-manager.component.scss']
})
export class DownloadManagerComponent extends UtilsService {

  public readonly supportedPlatforms = this.MASTER_SITES;

  @Input() url: string = '';
  @Output() urlChange = new EventEmitter<string>();
  @Output() analyze = new EventEmitter<'audio' | 'video' | 'gallery'>();
  @Output() download = new EventEmitter<void>();
  @Output() cancelDld = new EventEmitter<void>();

  constructor() {
    super();
    this.initializeIcons();
  }

  get status() {
    return this.tauriService.state().status;
  }

  override isVideoUrl(): boolean {
    return super.isVideoUrl(this.url);
  }

  override isGalleryUrl(): boolean {
    return super.isGalleryUrl(this.url);
  }
  async openSupportModal() {
    const modal = await this.modalCtrl.create({
      component: SupportGrid,
    });
    return await modal.present();
  }
}