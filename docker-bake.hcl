target "_common" {
  platforms = ["linux/amd64"]
}

variable "REGISTRY" {
  default = "git.agin.rocks/ctfilt-2"
}

variable "BRANCH" {
  default = "dev"
}

variable "COMMIT" {

}

variable "TAG" {
  default = equal(BRANCH, "master") ? "prod" : BRANCH
}

target "server" {
  context = "."
  inherits = ["_common"]
  args = {
    PROJECT_NAME = "server"
  }
  tags = ["${REGISTRY}/server:${TAG}", "${REGISTRY}/server:${COMMIT}"]
}

target "web" {
  context = "./web"
  inherits = ["_common"]
  tags = ["${REGISTRY}/web:${TAG}", "${REGISTRY}/web:${COMMIT}"]
}
