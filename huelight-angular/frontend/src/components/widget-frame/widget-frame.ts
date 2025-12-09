import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-widget-frame',
  imports: [],
  templateUrl: './widget-frame.html',
  styleUrl: './widget-frame.scss',
})
export class WidgetFrame {
  @Input() title: string = '';
  @Input() description: string = '';
}
