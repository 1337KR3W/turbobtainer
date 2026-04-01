import { Component } from '@angular/core';
import { IonHeader, IonToolbar, IonTitle } from "@ionic/angular/standalone";

@Component({
  selector: 'app-header',
  imports: [IonHeader, IonToolbar, IonTitle],
  templateUrl: './header.component.html',
  styleUrl: './header.component.scss',
})
export class Header {

}
