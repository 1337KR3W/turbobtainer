export type DownloadStatus = 'IDLE' | 'ANALYZING' | 'READY' | 'DOWNLOADING' | 'SUCCESS' | 'ERROR';

export interface AppState {
    status: DownloadStatus;
    mensaje?: string;
    videoTitle?: string;
    thumbnailUrl?: string;
    duration?: string;
    tipoSeleccionado?: 'audio' | 'video';
    progreso?: number;
}