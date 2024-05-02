import { ComponentFixture, TestBed } from '@angular/core/testing';

import { NightSkyComponent } from './night-sky.component';

describe('NightSkyComponent', () => {
  let component: NightSkyComponent;
  let fixture: ComponentFixture<NightSkyComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [NightSkyComponent]
    });
    fixture = TestBed.createComponent(NightSkyComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
