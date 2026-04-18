import { Component, Input, Output, EventEmitter } from '@angular/core';
import { CommonModule, DecimalPipe } from '@angular/common';
import {
  IonCard, IonCardHeader, IonCardTitle, IonCardSubtitle,
  IonCardContent, IonProgressBar, IonButton, IonIcon, IonItem, IonLabel
} from '@ionic/angular/standalone';

@Component({
  selector: 'app-download-status',
  standalone: true,
  imports: [
    CommonModule, DecimalPipe, IonCard, IonCardHeader, IonCardTitle,
    IonCardSubtitle, IonCardContent, IonProgressBar, IonButton,
    IonIcon, IonItem, IonLabel
  ],
  templateUrl: './download-status.component.html',
  styleUrls: ['./download-status.component.scss']
})
export class DownloadStatusComponent {
  @Input() state: any;
  @Output() cancel = new EventEmitter<void>();
}