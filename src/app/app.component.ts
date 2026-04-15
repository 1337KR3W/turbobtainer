import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { IonApp, IonContent, IonButton, IonIcon } from '@ionic/angular/standalone';
import { FooterComponent } from './components/footer/footer.component';
import { HeaderComponent } from "./components/header/header.component";
import { DownloadManagerComponent } from "./components/download-manager/download-manager.component";
import { SupportGrid } from './components/support-grid/support-grid';
import { UtilsService } from './services/utils.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    CommonModule, FormsModule, IonApp, IonContent,
    FooterComponent,
    HeaderComponent,
    DownloadManagerComponent,
    IonButton,
    IonIcon
  ],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent extends UtilsService {

  url: string = '';

  async analizar(tipo: 'audio' | 'video' | 'gallery') {
    if (!this.url) return;

    if (tipo === 'gallery') {
      await this.tauriService.obtenerMetadataGaleria(this.url);
    } else {
      await this.tauriService.obtenerMetadata(this.url, tipo);
    }
  }

  cancelar() {
    this.url = '';
    this.tauriService.reset();
  }
  iniciarDescarga() {
    this.tauriService.iniciarDescarga();
  }
  async openSupportModal() {
    const modal = await this.modalCtrl.create({
      component: SupportGrid,
    });
    return await modal.present();
  }
}