import { Component, Input, Output, EventEmitter, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { IonButton, IonCard, IonCardContent, IonCardHeader, IonCardSubtitle, IonCardTitle, IonCol, IonIcon, IonInput, IonItem, IonProgressBar, IonRow, IonSpinner, IonLabel, IonGrid, IonList, IonThumbnail, IonFooter } from '@ionic/angular/standalone';
import { UtilsService } from '../../services/utils.service';
import { SupportGrid } from '../support-grid/support-grid';
import { TauriService } from '../../services/tauri.service';
import { AnimeService } from '../../services/anime.service';
import { DomSanitizer } from '@angular/platform-browser';
import { VideoPlayerComponent } from "../video-player/video-player.component";
import { MetadataCardComponent } from "../metadata-card/metadata-card.component";

@Component({
  selector: 'app-download-manager',
  standalone: true,
  imports: [CommonModule, FormsModule, IonCard, IonCardHeader, IonCardTitle, IonCardContent,
    IonItem, IonInput, IonButton, IonIcon, IonRow, IonCol,
    IonSpinner, IonProgressBar, IonCardSubtitle, IonLabel, IonGrid, IonList, IonThumbnail, IonFooter, VideoPlayerComponent, MetadataCardComponent],
  templateUrl: './download-manager.component.html',
  styleUrls: ['./download-manager.component.scss']
})
export class DownloadManagerComponent {

  private readonly utils = inject(UtilsService);
  public readonly tauri = inject(TauriService);
  public animeService = inject(AnimeService);
  private readonly sanitizer = inject(DomSanitizer);
  public readonly supportedPlatforms = this.utils.MASTER_SITES;
  public viewMode: 'SEARCH' | 'EPISODES' | 'PLAYER' = 'SEARCH';
  public selectedAnime: any = null;

  @Input() url: string = '';
  @Output() urlChange = new EventEmitter<string>();
  @Output() analyze = new EventEmitter<'audio' | 'video' | 'gallery'>();
  @Output() download = new EventEmitter<void>();
  @Output() cancelDld = new EventEmitter<void>();
  public selectedEpisode: any = null;
  constructor() {

    this.utils.initializeIcons();
    this.utils.setRandomAscii();
  }

  get status() {
    return this.tauri.state().status;
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

  // Modificamos el método de búsqueda
  async handleSearch(event: any) {
    const query = event.target.value;
    if (query && query.length > 2 && !query.startsWith('http')) {
      // Si no es una URL, buscamos anime
      await this.animeService.searchAnime(query);
    }
  }



  async onInputChange(value: string) {
    this.url = value;
    this.urlChange.emit(value);

    if (!value || value.trim().length === 0) {
      this.animeService.reset();
      this.viewMode = 'SEARCH';
      return;
    }

    if (value.length > 2 && !value.startsWith('http')) {
      await this.animeService.searchAnime(value);
    }

    else if (value.startsWith('http')) {
      this.animeService.reset();
    }
  }

  goBack() {
    if (this.viewMode === 'PLAYER') {
      this.viewMode = 'EPISODES';
      // Opcional: limpiar el stream para que no se siga cargando en segundo plano
      this.animeService.resetStream();
    }
    else if (this.viewMode === 'EPISODES') {
      this.viewMode = 'SEARCH';
    }
  }

  async selectAnime(anime: any) {
    this.selectedAnime = anime;
    this.viewMode = 'EPISODES';
    await this.animeService.getEpisodes(anime.url);
  }

  // Seleccionar un episodio para reproducir
  async playEpisode(episode: any) {
    this.selectedEpisode = episode;
    this.viewMode = 'PLAYER';
    await this.animeService.getStream(episode.url);
  }

}