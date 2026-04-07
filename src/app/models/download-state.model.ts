export type DownloadStatus = 'IDLE' | 'ANALYZING' | 'READY' | 'DOWNLOADING' | 'SUCCESS' | 'ERROR';

export interface AppState {
    status: DownloadStatus;
    mensaje?: string;
    videoTitle?: string;
    tipoSeleccionado?: 'audio' | 'video';
    thumbnail?: string;
    duration?: string;
    size?: string;
    progreso?: number;
}