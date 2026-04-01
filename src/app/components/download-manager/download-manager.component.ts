import { Component, Input, Output, EventEmitter, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { IonicModule } from '@ionic/angular';
import { TauriService } from '../../services/tauri.service';

@Component({
  selector: 'app-download-manager',
  standalone: true,
  imports: [CommonModule, FormsModule, IonicModule],
  templateUrl: './download-manager.component.html',
  styleUrls: ['./download-manager.component.scss']
})
export class DownloadManagerComponent {
  public tauriService = inject(TauriService);

  @Input() url: string = '';
  @Output() urlChange = new EventEmitter<string>();
  @Output() onAnalizar = new EventEmitter<'audio' | 'video'>();
  @Output() onDownload = new EventEmitter<void>();
  @Output() onCancel = new EventEmitter<void>();
}