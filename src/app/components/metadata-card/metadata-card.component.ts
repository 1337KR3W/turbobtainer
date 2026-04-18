import { Component, Input, Output, EventEmitter } from '@angular/core';
import { CommonModule } from '@angular/common';
import {
  IonCard, IonGrid, IonRow, IonCol, IonList, IonItem,
  IonLabel, IonThumbnail, IonFooter, IonButton, IonIcon, IonBadge
} from '@ionic/angular/standalone';

@Component({
  selector: 'app-metadata-card',
  standalone: true,
  imports: [
    CommonModule, IonCard, IonGrid, IonRow, IonCol, IonList,
    IonItem, IonLabel, IonThumbnail, IonFooter, IonButton, IonIcon, IonBadge
  ],
  templateUrl: './metadata-card.component.html',
  styleUrls: ['./metadata-card.component.scss']
})
export class MetadataCardComponent {
  // Recibimos el estado de Tauri desde el padre
  @Input() state: any;

  // Eventos para comunicar acciones al padre
  @Output() download = new EventEmitter<void>();
  @Output() cancel = new EventEmitter<void>();
}