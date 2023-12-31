import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { SignupComponent } from './authentication/signup/signup.component';
import { LoginComponent } from './authentication/login/login.component';
import { HomeComponent } from './navbar-folder/home/home.component';
import { NavbarComponent } from './navbar/navbar.component';
import { RoadMapComponent } from './navbar-folder/road-map/road-map.component';
import { AdminComponent } from './admin/admin.component';
import { ReferenceFormComponent } from './navbar-folder/reference-form/reference-form.component';

const routes: Routes = [
  { path: '', redirectTo: '/login', pathMatch: 'full' },
  { path: 'login', component: LoginComponent},
  { path: 'signup', component: SignupComponent },
  { path: 'admin', component: AdminComponent },
  //{ path: 'home', component: HomeComponent },
  {
    path: 'home',
    component: NavbarComponent,
    children:[
      { path: '', component: HomeComponent },
      { path: 'roadmap', component: RoadMapComponent },
      { path: 'reference', component: ReferenceFormComponent },
    ],
  }
];
@NgModule({
  declarations: [],
  imports: [ RouterModule.forRoot(routes) ],
  exports: [ RouterModule ]
})
export class AppRoutingModule { }
