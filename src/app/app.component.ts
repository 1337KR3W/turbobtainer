import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { IonApp, IonContent, IonHeader, IonToolbar, IonTitle, IonInput, IonButton, IonItem, IonLabel } from '@ionic/angular/standalone';
import { invoke } from "@tauri-apps/api/core";

@Component({
  selector: "app-root",
  standalone: true,
  imports: [RouterOutlet, IonApp, IonContent, IonHeader, IonToolbar, IonTitle, IonInput, IonButton, IonItem, IonLabel],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent {
  greetingMessage = "";

  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;
    });
  }
}
