locals {
  subnet_ips = {
    public_a  = "172.31.48.0/24"
    public_b  = "172.31.49.0/24"
    private_a = "172.31.50.0/24"
    private_b = "172.31.51.0/24"
  }
}

resource "aws_default_vpc" "default" {}

data "aws_region" "current" {
}

resource "aws_subnet" "public_a" {
  vpc_id                  = aws_default_vpc.default.id
  cidr_block              = local.subnet_ips.public_a
  availability_zone       = "${data.aws_region.current.name}a"
  map_public_ip_on_launch = true
  tags = {
    Name = "public_a"
  }
}

resource "aws_route_table_association" "public_a_to_default" {
  route_table_id = aws_default_vpc.default.default_route_table_id
  subnet_id      = aws_subnet.public_a.id
}

resource "aws_subnet" "public_b" {
  vpc_id                  = aws_default_vpc.default.id
  cidr_block              = local.subnet_ips.public_b
  availability_zone       = "${data.aws_region.current.name}b"
  map_public_ip_on_launch = true
  tags = {
    Name = "public_b"
  }
}

resource "aws_route_table_association" "public_b_to_default" {
  route_table_id = aws_default_vpc.default.default_route_table_id
  subnet_id      = aws_subnet.public_b.id
}

resource "aws_route_table" "private" {
  vpc_id = aws_default_vpc.default.id
}

resource "aws_subnet" "private_a" {
  vpc_id                  = aws_default_vpc.default.id
  availability_zone       = "${data.aws_region.current.name}a"
  cidr_block              = local.subnet_ips.private_a
  map_public_ip_on_launch = false
  tags = {
    Name = "private_a"
  }
}

resource "aws_route_table_association" "private_to_private" {
  route_table_id = aws_route_table.private.id
  subnet_id      = aws_subnet.private_a.id
}

resource "aws_subnet" "private_b" {
  vpc_id            = aws_default_vpc.default.id
  availability_zone = "${data.aws_region.current.name}b"
  cidr_block        = local.subnet_ips.private_b
  tags = {
    Name = "private_b"
  }
}

resource "aws_route_table_association" "private_b_to_private" {
  route_table_id = aws_route_table.private.id
  subnet_id      = aws_subnet.private_b.id
}

resource "aws_security_group" "load_balancer" {
  name        = "${var.namespace}-${var.project_name}-alb"
  description = "Allow traffic to loadbalancer"
  vpc_id      = aws_default_vpc.default.id

  ingress {
    protocol    = "tcp"
    from_port   = 80
    to_port     = 80
    cidr_blocks = ["0.0.0.0/0"]
    self        = false
  }

  egress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }
}

resource "aws_lb" "load_balancer" {
  name               = "${var.namespace}-${var.project_name}-alb"
  load_balancer_type = "application"
  internal           = false
  subnets = [
    aws_subnet.public_a.id,
    aws_subnet.public_b.id
  ]
  security_groups = [aws_security_group.load_balancer.id]
}