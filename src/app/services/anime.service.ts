import { Injectable, signal, computed } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { Anime, Episode, StreamSource, AnimeState } from '../models/anime.model';

@Injectable({
    providedIn: 'root'
})
export class AnimeService {
    // Estado privado con Signals
    private readonly _state = signal<AnimeState>({
        status: 'IDLE',
        results: [],
        episodes: [],
        currentStream: null
    });

    // Exposición de estados como solo lectura para los componentes
    public state = this._state.asReadonly();
    public results = computed(() => this._state().results);
    public episodes = computed(() => this._state().episodes);

    constructor() { }

    /**
     * Busca animes por texto usando el motor de Rust
     */
    async searchAnime(query: string) {
        if (!query.trim()) return;

        this._state.update(s => ({ ...s, status: 'SEARCHING', results: [] }));

        try {
            const results = await invoke<Anime[]>('search_anime', { query });
            this._state.update(s => ({ ...s, status: 'IDLE', results }));
        } catch (error) {
            this._state.update(s => ({ ...s, status: 'ERROR', error: error as string }));
        }
    }

    /**
     * Obtiene la lista de episodios de una serie
     */
    async getEpisodes(animeUrl: string) {
        this._state.update(s => ({ ...s, status: 'LOADING_EPISODES', episodes: [] }));

        try {
            const episodes = await invoke<Episode[]>('get_anime_episodes', { url: animeUrl });
            this._state.update(s => ({ ...s, status: 'IDLE', episodes }));
        } catch (error) {
            this._state.update(s => ({ ...s, status: 'ERROR', error: error as string }));
        }
    }

    /**
     * Extrae el link de video directo de un episodio
     */
    async getStream(episodeUrl: string) {
        this._state.update(s => ({ ...s, status: 'GETTING_STREAM', currentStream: null }));

        try {
            const stream = await invoke<StreamSource>('get_stream_link', { url: episodeUrl });
            this._state.update(s => ({ ...s, status: 'READY', currentStream: stream }));
        } catch (error) {
            this._state.update(s => ({ ...s, status: 'ERROR', error: error as string }));
        }
    }

    reset() {
        this._state.set({
            status: 'IDLE',
            results: [],
            episodes: [],
            currentStream: null
        });
    }
    resetStream() {
        this._state.update(s => ({ ...s, currentStream: null, status: 'IDLE' }));
    }


}