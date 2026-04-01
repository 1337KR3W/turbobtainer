import { Component, Input, Output, EventEmitter, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { IonicModule } from '@ionic/angular';
import { TauriService } from '../../services/tauri.service';
import { IonButton, IonCard, IonCardContent, IonCardHeader, IonCardSubtitle, IonCardTitle, IonCol, IonIcon, IonInput, IonItem, IonProgressBar, IonRow, IonSpinner, IonText } from '@ionic/angular/standalone';

@Component({
  selector: 'app-download-manager',
  standalone: true,
  imports: [CommonModule, FormsModule, IonicModule, IonCard, IonCardHeader, IonCardTitle, IonCardContent,
    IonItem, IonInput, IonButton, IonIcon, IonRow, IonCol,
    IonSpinner, IonProgressBar, IonText, IonCardSubtitle],
  templateUrl: './download-manager.component.html',
  styleUrls: ['./download-manager.component.scss']
})
export class DownloadManagerComponent {
  public tauriService = inject(TauriService);

  @Input() url: string = '';
  @Output() urlChange = new EventEmitter<string>();
  @Output() onAnalizar = new EventEmitter<'audio' | 'video'>();
  @Output() onDownload = new EventEmitter<void>();
  @Output() onCancel = new EventEmitter<void>();
}