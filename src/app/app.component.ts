import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { TauriService } from './services/tauri.service';
import {
  IonApp, IonContent, IonHeader, IonToolbar, IonTitle,
  IonInput, IonButton, IonItem, IonLabel, IonIcon, IonCard,
  IonCardContent, IonRow, IonCol, IonSpinner, IonText, IonCardHeader,
  IonCardTitle, IonCardSubtitle, IonProgressBar
} from '@ionic/angular/standalone';

import { addIcons } from 'ionicons';
import {
  musicalNotesOutline,
  videocamOutline,
  checkmarkDoneOutline,
  alertCircleOutline,
  trashOutline,
  cloudDownloadOutline,
  logoYoutube
} from 'ionicons/icons';
import { FooterComponent } from './components/footer/footer.component';
import { Header } from "./components/header/header.component";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    CommonModule, FormsModule, IonApp, IonContent, IonHeader,
    IonToolbar, IonTitle, IonInput, IonButton, IonItem,
    IonLabel, IonIcon, IonCard, IonCardContent, IonRow,
    IonCol, IonSpinner, IonText,
    IonCardHeader,
    IonCardTitle,
    IonCardSubtitle,
    IonProgressBar,
    FooterComponent,
    Header
  ],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  // ✅ Inyectamos el servicio para acceder a su estado (state)
  public tauriService = inject(TauriService);

  // La URL es lo único que mantenemos local porque pertenece al formulario activo
  url: string = '';

  constructor() {
    addIcons({
      musicalNotesOutline,
      videocamOutline,
      checkmarkDoneOutline,
      alertCircleOutline,
      trashOutline,
      cloudDownloadOutline,
      logoYoutube
    });
  }

  /**
   * Acción para los botones de Audio/Video
   */
  analizar(tipo: 'audio' | 'video') {
    if (this.url.trim()) {
      this.tauriService.obtenerMetadata(this.url, tipo);
    }
  }

  /**
   * Acción para resetear y volver al inicio
   */
  cancelar() {
    this.url = ''; // Limpiamos el input
    this.tauriService.reset(); // Movemos el estado a 'IDLE'
  }
  iniciarDescarga() {
    this.tauriService.iniciarDescarga();
  }
}