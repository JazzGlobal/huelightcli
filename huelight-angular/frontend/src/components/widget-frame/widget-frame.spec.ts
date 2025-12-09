import { ComponentFixture, TestBed } from '@angular/core/testing';

import { WidgetFrame } from './widget-frame';

describe('WidgetFrame', () => {
  let component: WidgetFrame;
  let fixture: ComponentFixture<WidgetFrame>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [WidgetFrame]
    })
    .compileComponents();

    fixture = TestBed.createComponent(WidgetFrame);
    component = fixture.componentInstance;
    await fixture.whenStable();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
