export type DownloadStatus = 'IDLE' | 'ANALYZING' | 'READY' | 'DOWNLOADING' | 'SUCCESS' | 'ERROR';

export interface AppState {
    status: DownloadStatus;
    mensaje?: string;
    videoTitle?: string;
    tipoSeleccionado?: 'audio' | 'video' | 'gallery';
    thumbnail?: string;
    duration?: string;
    size?: string;
    imageCount?: number;
    progreso?: number;
}