variable "namespace" {
  type    = string
  default = "production"
}

variable "project_name" {
  type    = string
  default = "url-shortener"
}

variable "db_password" {
  description = "Postgres password"
  type        = string
  sensitive   = true
}
