export type DownloadStatus = 'IDLE' | 'ANALYZING' | 'READY' | 'DOWNLOADING' | 'SUCCESS' | 'ERROR';

export interface AppState {
    status: DownloadStatus;
    message?: string;
    videoTitle?: string;
    selectedType?: 'audio' | 'video' | 'gallery';
    thumbnail?: string;
    sourceLogo?: string;
    duration?: string;
    size?: string;
    hasPlaylist?: boolean;
    shouldDownloadPlaylist?: boolean;
    imageCount?: number;
    progress?: number;
}