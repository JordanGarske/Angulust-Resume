import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppComponent } from './app.component';
import { HttpClientModule } from '@angular/common/http';
import { SignupComponent } from './authentication/signup/signup.component';
import { LoginComponent } from './authentication/login/login.component';
import { AppRoutingModule } from './app-routing.module'; 
import { HomeComponent } from './navbar-folder/home/home.component';

//angular material 
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { FormsModule } from '@angular/forms';
import { NavbarComponent } from './navbar/navbar.component';
import { RoadMapComponent } from './navbar-folder/road-map/road-map.component';
import { AdminComponent } from './admin/admin.component';
import { ReferenceFormComponent } from './navbar-folder/reference-form/reference-form.component';
import { ResumeQualificationComponent } from './navbar-folder/home/resume-qualification/resume-qualification.component';
import { ProjectsComponent } from './navbar-folder/projects/projects.component';
import { ReferencesComponent } from './navbar-folder/home/references/references.component';
@NgModule({
  declarations: [
    AppComponent,
    SignupComponent,
    LoginComponent,
    HomeComponent,
    NavbarComponent,
    RoadMapComponent,
    AdminComponent,
    ReferenceFormComponent,
    ResumeQualificationComponent,
    ProjectsComponent,
    ReferencesComponent
  ],
  imports: [
    BrowserModule,
    HttpClientModule,
    FormsModule,
    BrowserAnimationsModule,
    AppRoutingModule,
    HttpClientModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
