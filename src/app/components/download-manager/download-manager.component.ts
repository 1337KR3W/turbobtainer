import { Component, Input, Output, EventEmitter, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { TauriService } from '../../services/tauri.service';
import { IonButton, IonCard, IonCardContent, IonCardHeader, IonCardSubtitle, IonCardTitle, IonCol, IonIcon, IonInput, IonItem, IonProgressBar, IonRow, IonSpinner, IonLabel, IonGrid, IonList, IonThumbnail, IonFooter } from '@ionic/angular/standalone';

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
  public tauriService = inject(TauriService);

  @Input() url: string = '';
  @Output() urlChange = new EventEmitter<string>();
  @Output() analizar = new EventEmitter<'audio' | 'video'>();
  @Output() download = new EventEmitter<void>();
  @Output() cancelDld = new EventEmitter<void>();
}