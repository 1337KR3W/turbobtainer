import { Component, Input, Output, EventEmitter, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DomSanitizer, SafeResourceUrl } from '@angular/platform-browser';
import { IonButton, IonIcon, IonCard, IonItem, IonLabel, IonBadge, IonCardContent, IonSpinner, IonList, IonThumbnail, IonGrid, IonRow, IonCol } from '@ionic/angular/standalone';
import { StreamSource } from '../../models/anime.model';
import { AnimeService } from '../../services/anime.service';

@Component({
  selector: 'app-video-player',
  standalone: true,
  imports: [CommonModule, IonButton, IonIcon, IonCard, IonItem, IonLabel, IonBadge, IonCardContent, IonSpinner, IonList, IonThumbnail, IonGrid, IonRow, IonCol],
  templateUrl: './video-player.component.html',
  styleUrls: ['./video-player.component.scss']
})
export class VideoPlayerComponent {
  private readonly sanitizer = inject(DomSanitizer);
  public readonly animeService = inject(AnimeService);

  ;
  public viewMode: 'SEARCH' | 'EPISODES' | 'PLAYER' = 'SEARCH';
  public selectedAnime: any = null;

  @Input() url: string = '';
  @Output() urlChange = new EventEmitter<string>();
  @Output() analyze = new EventEmitter<'audio' | 'video' | 'gallery'>();
  @Output() download = new EventEmitter<void>();
  @Output() cancelDld = new EventEmitter<void>();
  public selectedEpisode: any = null;

  // Definimos qué datos puede recibir de download-manager
  @Input() stream: StreamSource | null = null;
  @Input() animeTitle: string = '';
  @Input() episodeNumber: string | number = '';

  // Definimos el evento para avisar que queremos cerrar el video
  @Output() back = new EventEmitter<void>();

  // La lógica del SafeUrl se queda AQUÍ, liberando al componente principal
  getSafeUrl(): SafeResourceUrl | null {
    if (this.stream?.url) {
      return this.sanitizer.bypassSecurityTrustResourceUrl(this.stream.url);
    }
    return null;
  }
  onBackClick() {
    this.back.emit();
  }

  // Método para seleccionar un Anime y ver sus capítulos
  async selectAnime(anime: any) {
    this.selectedAnime = anime;
    this.viewMode = 'EPISODES';
    await this.animeService.getEpisodes(anime.url);
  }

  // Método para reproducir un episodio
  async playEpisode(episode: any) {
    this.selectedEpisode = episode; // Guardamos el episodio seleccionado
    this.viewMode = 'PLAYER';
    await this.animeService.getStream(episode.url);
  }

  // Volver atrás
  public goBack() {
    if (this.viewMode === 'PLAYER') {
      // Si estamos en el video, volvemos a la lista de episodios
      this.viewMode = 'EPISODES';
      // Limpiamos el stream actual para que el iframe deje de existir
      this.animeService.resetStream(); // Opcional, pero recomendado
    }
    else if (this.viewMode === 'EPISODES') {
      // Si estamos en episodios, volvemos a los resultados de búsqueda
      this.viewMode = 'SEARCH';
    }
  }
}