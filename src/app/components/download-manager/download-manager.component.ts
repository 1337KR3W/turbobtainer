import { Component, Input, Output, EventEmitter, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { IonButton, IonCard, IonCardContent, IonCardHeader, IonCardSubtitle, IonCardTitle, IonCol, IonIcon, IonInput, IonItem, IonProgressBar, IonRow, IonSpinner, IonLabel, IonGrid, IonList, IonThumbnail, IonFooter } from '@ionic/angular/standalone';
import { UtilsService } from '../../services/utils.service';
import { SupportGrid } from '../support-grid/support-grid';
import { TauriService } from '../../services/tauri.service';
import { AnimeService } from '../../services/anime.service';

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

  private readonly utils = inject(UtilsService);
  public readonly tauri = inject(TauriService);

  public readonly supportedPlatforms = this.utils.MASTER_SITES;

  @Input() url: string = '';
  @Output() urlChange = new EventEmitter<string>();
  @Output() analyze = new EventEmitter<'audio' | 'video' | 'gallery'>();
  @Output() download = new EventEmitter<void>();
  @Output() cancelDld = new EventEmitter<void>();

  constructor(private readonly animeService: AnimeService) {

    this.utils.initializeIcons();
    this.utils.setRandomAscii();
  }

  get status() {
    return this.tauri.state().status;
  }

  async pruebaAnime() {
    console.log('--- PASO 1: Buscando Anime ---');
    await this.animeService.searchAnime('Solo Leveling');

    const resultados = this.animeService.results();
    console.log('Resultados encontrados:', resultados);

    if (resultados.length > 0) {
      const primeraSerie = resultados[1]; // Usamos resultados[1] para la Temporada 1 como vimos antes
      console.log(`--- PASO 2: Obteniendo capítulos de: ${primeraSerie.title} ---`);

      await this.animeService.getEpisodes(primeraSerie.url);
      const episodios = this.animeService.episodes();
      console.log('LISTA DE EPISODIOS EXTRAÍDA:', episodios);

      if (episodios.length > 0) {
        console.log('✅ TEST EXITOSO (Capítulos)');

        // --- AQUÍ EMPIEZA EL PASO 3 (EL NUEVO) ---
        const primerEpisodio = episodios[0];
        console.log(`--- PASO 3: Extrayendo link de video del cap: ${primerEpisodio.number} ---`);
        console.log('URL del episodio:', primerEpisodio.url);

        // Llamamos a la nueva función que acabas de poner en Rust
        await this.animeService.getStream(primerEpisodio.url);

        const stream = this.animeService.state().currentStream;
        if (stream) {
          console.log('✅ TEST EXITOSO (Stream): Servidor y Link detectados:', stream);
        } else {
          console.log('❌ TEST FALLIDO (Stream): No se pudo obtener el servidor.');
        }
        // ----------------------------------------

      } else {
        console.log('❌ TEST FALLIDO: La lista de episodios está vacía.');
      }
    } else {
      console.log('⚠️ No se encontraron series.');
    }
  }

  isVideoUrl(): boolean {
    return this.utils.isVideoUrl(this.url);
  }

  isGalleryUrl(): boolean {
    return this.utils.isGalleryUrl(this.url);
  }
  async openSupportModal() {
    const modal = await this.utils.modalCtrl.create({
      component: SupportGrid,
    });
    return await modal.present();
  }
  setRandomBackGround() {
    this.utils.setRandomAscii();
  }

}