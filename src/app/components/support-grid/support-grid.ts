import { Component } from '@angular/core';
import { IonContent, IonGrid, IonRow, IonCol, IonIcon, IonHeader, IonToolbar, IonTitle, IonButtons, IonButton, IonSearchbar } from "@ionic/angular/standalone";
import { UtilsService } from '../../services/utils.service';
@Component({
  selector: 'app-support-grid',
  standalone: true,
  imports: [IonContent, IonGrid, IonRow, IonCol, IonIcon, IonHeader, IonToolbar, IonTitle, IonButtons, IonButton, IonSearchbar],
  templateUrl: './support-grid.html',
  styleUrl: './support-grid.scss',
})
export class SupportGrid extends UtilsService {

  public sitiosSoportados: string[] = [...this.MASTER_SITES];

  constructor() {
    super();
    this.initializeIcons();
    console.log(this.MASTER_SITES.length);
  }

  filterSites(event: any) {
    const query = event.target.value.toLowerCase().trim();
    if (!query) {
      this.sitiosSoportados = [...this.MASTER_SITES];
      return;
    }
    this.sitiosSoportados = this.MASTER_SITES.filter(s => s.toLowerCase().includes(query));
  }

  handleImageError(event: any) {
    event.target.src = 'assets/icon/default-web.svg';
  }

  dismiss() {
    this.modalCtrl.dismiss();
  }

}
