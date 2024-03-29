import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RoadMapComponent } from './road-map.component';

describe('RoadMapComponent', () => {
  let component: RoadMapComponent;
  let fixture: ComponentFixture<RoadMapComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [RoadMapComponent]
    });
    fixture = TestBed.createComponent(RoadMapComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
