# ⭐ Star Dodge

**Star Dodge** é um mini–jogo criado usando **Rust** e **Bevy Engine** com o objetivo principal de estudar desenvolvimento de jogos, ECS e arquitetura de sistemas no ecossistema Rust.

No jogo, você controla uma **bola azul** enquanto:

- **Inimigos (bolas vermelhas)** vão surgindo ao longo do tempo.
- **Estrelas** aparecem periodicamente no mapa e você precisa coletá-las.

O objetivo é **coletar o máximo de estrelas possível sem ser atingido por nenhum inimigo**.  
Se um inimigo encostar em você: game over.

---

## 🎮 Funcionalidades

- Jogador controlando uma bola azul em 2D  
- Spawn progressivo de inimigos  
- Spawn aleatório de estrelas  
- Sistema simples de colisão  
- Contador de estrelas coletadas  
- Lógica de game over  

---

## 🛠️ Tecnologias Utilizadas

- **Rust**
- **Bevy Engine** (ECS, 2D, time, systems)
- **Cargo** para build e gerência do projeto

---

## 🎯 Objetivo do Projeto

Este projeto foi criado com foco em **aprendizado**, explorando:

- Estrutura ECS (Entidades, Componentes e Sistemas)
- Loops de jogo e agendamento de sistemas
- Spawn temporizado de entidades
- Movimentação no plano 2D
- Detecção de colisões simples
- Organização de um projeto de jogo com Bevy

---

## ▶️ Rodando o Projeto

Certifique-se de ter o Rust instalado.

```sh
cargo run
