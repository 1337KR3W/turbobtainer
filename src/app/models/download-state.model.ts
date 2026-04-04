export type DownloadStatus = 'IDLE' | 'ANALYZING' | 'READY' | 'DOWNLOADING' | 'SUCCESS' | 'ERROR';

export interface AppState {
    status: DownloadStatus;
    mensaje?: string;
    videoTitle?: string;
    thumbnailUrl?: string;
    tipoSeleccionado?: 'audio' | 'video';
    progreso?: number;
}