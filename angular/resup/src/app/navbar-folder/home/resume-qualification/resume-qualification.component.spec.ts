import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ResumeQualificationComponent } from './resume-qualification.component';

describe('ResumeQualificationComponent', () => {
  let component: ResumeQualificationComponent;
  let fixture: ComponentFixture<ResumeQualificationComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [ResumeQualificationComponent]
    });
    fixture = TestBed.createComponent(ResumeQualificationComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
