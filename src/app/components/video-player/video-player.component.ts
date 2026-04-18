import { Component, Input, Output, EventEmitter, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DomSanitizer, SafeResourceUrl } from '@angular/platform-browser';
import { IonButton, IonIcon, IonCard, IonItem, IonLabel, IonBadge, IonCardContent, IonSpinner } from '@ionic/angular/standalone';
import { StreamSource } from '../../models/anime.model';

@Component({
  selector: 'app-video-player',
  standalone: true,
  imports: [CommonModule, IonButton, IonIcon, IonCard, IonItem, IonLabel, IonBadge, IonCardContent, IonSpinner],
  templateUrl: './video-player.component.html',
  styleUrls: ['./video-player.component.scss']
})
export class VideoPlayerComponent {
  private readonly sanitizer = inject(DomSanitizer);

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
}