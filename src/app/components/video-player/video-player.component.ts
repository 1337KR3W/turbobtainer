import { Component, ElementRef, ViewChild, AfterViewInit, OnDestroy, effect, inject } from '@angular/core';
import videojs from 'video.js';
import { AnimeService } from '../../services/anime.service';

@Component({
  selector: 'app-video-player',
  standalone: true,
  templateUrl: './video-player.component.html',
  styleUrls: ['./video-player.component.scss']
})
export class VideoPlayerComponent implements AfterViewInit, OnDestroy {
  @ViewChild('videoPlayer') videoElement!: ElementRef;
  private readonly animeService = inject(AnimeService);
  player: any;

  constructor() {
    // Escuchamos los cambios en el stream mediante un effect
    effect(() => {
      const stream = this.animeService.state().currentStream;
      if (stream && this.player) {
        this.player.src({
          src: stream.url,
          type: stream.url.includes('m3u8') ? 'application/x-mpegURL' : 'video/mp4'
        });
        this.player.play();
      }
    });
  }

  ngAfterViewInit() {
    this.player = videojs(this.videoElement.nativeElement, {
      fluid: true,
      autoplay: false,
      controls: true
    });
  }

  ngOnDestroy() {
    if (this.player) {
      this.player.dispose();
    }
  }
}