import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { TauriService } from './services/tauri.service';
import { IonApp, IonContent } from '@ionic/angular/standalone';

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
import { HeaderComponent } from "./components/header/header.component";
import { DownloadManagerComponent } from "./components/download-manager/download-manager.component";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    CommonModule, FormsModule, IonApp, IonContent,
    FooterComponent,
    HeaderComponent,
    DownloadManagerComponent
  ],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {

  public tauriService = inject(TauriService);

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
   * Acción para los botones de Audio/Video/Gallery
   */
  async analizar(tipo: 'audio' | 'video' | 'gallery') {
    if (!this.url) return;

    if (tipo === 'gallery') {
      await this.tauriService.obtenerMetadataGaleria(this.url);
    } else {
      await this.tauriService.obtenerMetadata(this.url, tipo);
    }
  }

  /**
   * Acción para resetear y volver al inicio
   */
  cancelar() {
    this.url = '';
    this.tauriService.reset();
  }
  iniciarDescarga() {
    this.tauriService.iniciarDescarga();
  }
}