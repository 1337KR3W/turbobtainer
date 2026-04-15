import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SupportGrid } from './support-grid';

describe('SupportGrid', () => {
  let component: SupportGrid;
  let fixture: ComponentFixture<SupportGrid>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SupportGrid]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SupportGrid);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
