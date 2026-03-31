import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common'; // Necesario para *ngIf
import { invoke } from '@tauri-apps/api/core';
import {
  IonApp, IonContent, IonHeader, IonToolbar,
  IonTitle, IonInput, IonButton, IonItem,
  IonLabel, IonIcon, IonCard, IonCardContent,
  IonRow, IonCol, IonSpinner, IonCardHeader,
  IonCardTitle, IonCardSubtitle
} from '@ionic/angular/standalone';
import { addIcons } from 'ionicons'; // Para que los iconos se vean
import { musicalNotesOutline, videocamOutline, logoYoutube, informationCircleOutline } from 'ionicons/icons';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    CommonModule, FormsModule, IonApp, IonContent, IonHeader,
    IonToolbar, IonTitle, IonInput, IonButton, IonItem,
    IonLabel, IonIcon, IonCard, IonCardContent, IonRow,
    IonCol, IonSpinner, IonCardHeader, IonCardTitle, IonCardSubtitle
  ],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  url: string = '';
  titulo: string = '';
  loading: boolean = false;
  mensajeError: string = '';

  constructor() {
    // Registramos los iconos para que Ionic los renderice
    addIcons({ musicalNotesOutline, videocamOutline, logoYoutube, informationCircleOutline });
  }

  // Esta es la función que deben llamar tus botones en el HTML
  async procesarEnlace(tipo: 'audio' | 'video') {
    if (!this.url) return;

    this.loading = true;
    this.titulo = '';
    this.mensajeError = '';

    try {
      // Llamamos a Rust (PMO-6)
      const resultado = await invoke<string>('check_video_url', { url: this.url });
      this.titulo = resultado;
      console.log(`Modo ${tipo} preparado para: ${this.titulo}`);
    } catch (error) {
      this.mensajeError = error as string;
    } finally {
      this.loading = false;
    }
  }

  limpiarEstado() {
    this.titulo = '';
    this.mensajeError = '';
  }
}