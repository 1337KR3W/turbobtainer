import { Component, Input, Output, EventEmitter, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { IonButton, IonCard, IonCardContent, IonCardHeader, IonCardSubtitle, IonCardTitle, IonCol, IonIcon, IonInput, IonItem, IonProgressBar, IonRow, IonSpinner, IonLabel, IonGrid, IonList, IonThumbnail, IonFooter } from '@ionic/angular/standalone';
import { UtilsService } from '../../services/utils.service';
import { SupportGrid } from '../support-grid/support-grid';
import { TauriService } from '../../services/tauri.service';
import { AnimeService } from '../../services/anime.service';
import { DomSanitizer, SafeResourceUrl } from '@angular/platform-browser';
import { VideoPlayerComponent } from "../video-player/video-player.component";

@Component({
  selector: 'app-download-manager',
  standalone: true,
  imports: [CommonModule, FormsModule, IonCard, IonCardHeader, IonCardTitle, IonCardContent,
    IonItem, IonInput, IonButton, IonIcon, IonRow, IonCol,
    IonSpinner, IonProgressBar, IonCardSubtitle, IonLabel, IonGrid, IonList, IonThumbnail, IonFooter, VideoPlayerComponent],
  templateUrl: './download-manager.component.html',
  styleUrls: ['./download-manager.component.scss']
})
export class DownloadManagerComponent {

  private readonly utils = inject(UtilsService);
  public readonly tauri = inject(TauriService);
  public animeService = inject(AnimeService);
  private readonly sanitizer = inject(DomSanitizer);
  public readonly supportedPlatforms = this.utils.MASTER_SITES;

  @Input() url: string = '';
  @Output() urlChange = new EventEmitter<string>();
  @Output() analyze = new EventEmitter<'audio' | 'video' | 'gallery'>();
  @Output() download = new EventEmitter<void>();
  @Output() cancelDld = new EventEmitter<void>();

  constructor() {

    this.utils.initializeIcons();
    this.utils.setRandomAscii();
  }

  get status() {
    return this.tauri.state().status;
  }

  getSafeUrl(): SafeResourceUrl | null {
    const stream = this.animeService.state().currentStream;
    if (stream && stream.url) {
      return this.sanitizer.bypassSecurityTrustResourceUrl(stream.url);
    }
    return null;
  }

  async pruebaAnime() {
    console.log('--- PASO 1: Buscando Anime ---');
    await this.animeService.searchAnime('Solo Leveling');

    const resultados = this.animeService.results();
    console.log('Resultados encontrados:', resultados);

    if (resultados.length > 0) {
      // Usamos el índice [1] como acordamos para evitar la versión coreana/live action si aplica
      const primeraSerie = resultados[1] || resultados[0];
      console.log(`--- PASO 2: Obteniendo capítulos de: ${primeraSerie.title} ---`);

      await this.animeService.getEpisodes(primeraSerie.url);
      const episodios = this.animeService.episodes();
      console.log('LISTA DE EPISODIOS EXTRAÍDA:', episodios);

      if (episodios.length > 0) {
        console.log('✅ TEST EXITOSO (Capítulos)');

        const primerEpisodio = episodios[0];
        console.log(`--- PASO 3: Extrayendo video de: Cap ${primerEpisodio.number} ---`);

        // Ejecutamos la lógica de Rust
        await this.animeService.getStream(primerEpisodio.url);

        // Obtenemos el resultado del estado
        const stream = this.animeService.state().currentStream;

        if (stream && stream.url) {
          if (stream.server === 'Streamwish_Direct') {
            console.log('🔥 ¡SÚPER ÉXITO! Link directo obtenido:', stream.url);
            console.log('Este link debería cargar directamente en Video.js');
          } else {
            console.log('✅ TEST EXITOSO (Modo Iframe):', stream);
            console.log('Cargando el reproductor embebido original...');
          }
        } else {
          console.error('❌ TEST FALLIDO: El servicio devolvió un objeto vacío o nulo.');
        }

      } else {
        console.log('❌ TEST FALLIDO: No hay episodios disponibles.');
      }
    } else {
      console.log('⚠️ No se encontraron series en la búsqueda.');
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