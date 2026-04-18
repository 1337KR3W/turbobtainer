import { Component, Input, Output, EventEmitter } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import {
  IonCard, IonCardHeader, IonCardTitle, IonCardContent, IonItem,
  IonInput, IonRow, IonCol, IonButton, IonIcon, IonSpinner
} from '@ionic/angular/standalone';

@Component({
  selector: 'app-search-manager',
  standalone: true,
  imports: [
    CommonModule, FormsModule, IonCard, IonCardHeader, IonCardTitle,
    IonCardContent, IonItem, IonInput, IonRow, IonCol, IonButton,
    IonIcon, IonSpinner
  ],
  templateUrl: './search-manager.component.html',
  styleUrls: ['./search-manager.component.scss']
})
export class SearchManagerComponent {
  @Input() url: string = '';
  @Input() status: string = 'IDLE';
  @Input() tipoSeleccionado: "audio" | "video" | "gallery" | string | null | undefined = null;
  @Input() animeCount: number = 0;

  // Helpers para validación (pasados desde el padre o lógica simple)
  @Input() isVideo: boolean = false;
  @Input() isGallery: boolean = false;

  @Output() urlChange = new EventEmitter<string>();
  @Output() analyze = new EventEmitter<'audio' | 'video' | 'gallery'>();
  @Output() openAnime = new EventEmitter<void>();
  @Output() openSupport = new EventEmitter<void>();
  @Output() randomBg = new EventEmitter<void>();
}