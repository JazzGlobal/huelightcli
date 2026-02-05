import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { WidgetFrame } from '../components/widget-frame/widget-frame';
import { LightService, LightDto } from '../services/lightservice';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, WidgetFrame],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})

export class App {
  protected readonly title = signal('frontend');
  readonly lights = signal(<LightDto[]>[]);
  constructor(private lightService: LightService) {
    this.lightService.getLights().subscribe((data) => {
      this.lights.set(data);
    });
  }
}
