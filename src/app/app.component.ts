import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { IonApp, IonContent } from '@ionic/angular/standalone';
import { FooterComponent } from './components/footer/footer.component';
import { HeaderComponent } from "./components/header/header.component";
import { DownloadManagerComponent } from "./components/download-manager/download-manager.component";
import { UtilsService } from './services/utils.service';
import { TauriService } from './services/tauri.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    CommonModule, FormsModule, IonApp, IonContent,
    FooterComponent,
    HeaderComponent,
    DownloadManagerComponent,
  ],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {

  private readonly utils = inject(UtilsService);
  private readonly tauri = inject(TauriService);

  url: string = '';
  constructor(public utilsService: UtilsService) { }
  async checkUrlType(tipo: 'audio' | 'video' | 'gallery') {
    if (!this.url) return;

    if (tipo === 'gallery') {
      await this.tauri.getMetadataGallery(this.url);
    } else {
      await this.tauri.getMetadata(this.url, tipo);
    }
  }

  cancel() {
    this.url = '';
    this.tauri.reset();
  }
  startDownload() {
    this.tauri.startDownload();
  }


}