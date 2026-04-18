import { Component, Input, Output, EventEmitter } from '@angular/core';
import { CommonModule } from '@angular/common';
import {
  IonGrid, IonRow, IonCol, IonCard, IonCardHeader, IonCardSubtitle,
  IonItem, IonButton, IonIcon, IonLabel, IonSpinner
} from '@ionic/angular/standalone';
import { VideoPlayerComponent } from '../video-player/video-player.component';

@Component({
  selector: 'app-anime-manager',
  standalone: true,
  imports: [
    CommonModule, IonGrid, IonRow, IonCol, IonCard, IonCardHeader,
    IonCardSubtitle, IonItem, IonButton, IonIcon, IonLabel, IonSpinner,
    VideoPlayerComponent
  ],
  templateUrl: './anime-manager.component.html',
  styleUrls: ['./anime-manager.component.scss']
})
export class AnimeManagerComponent {
  @Input() viewMode: string = 'SEARCH';
  @Input() results: any[] = [];
  @Input() episodes: any[] = [];
  @Input() selectedAnime: any = null;
  @Input() selectedEpisode: any = null;
  @Input() currentStream: any = null;
  @Input() status: string = '';

  @Output() selectAnime = new EventEmitter<any>();
  @Output() playEpisode = new EventEmitter<any>();
  @Output() goBack = new EventEmitter<void>();
}