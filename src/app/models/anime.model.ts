export interface Anime {
    title: string;
    url: string;        // URL de la página de la serie en la fuente
    thumbnail: string;
    description?: string;
}

export interface Episode {
    number: string;
    url: string;        // URL de la página del episodio
}

export interface StreamSource {
    server: string;     // Nombre del servidor (ej: 'Vidoza', 'Streamtape')
    url: string;        // URL real del video (.mp4 o .m4u8)
    quality?: string;
}

export interface AnimeState {
    status: 'IDLE' | 'SEARCHING' | 'LOADING_EPISODES' | 'GETTING_STREAM' | 'READY' | 'ERROR';
    results: Anime[];
    episodes: Episode[];
    currentStream: StreamSource | null;
    error?: string;
}