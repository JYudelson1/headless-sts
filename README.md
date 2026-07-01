# Headless StS

This is a headless version of Slay the Spire (TM MegaCrit, it's all theirs, not mine at all). The goal is to make a version that runs quickly enough to train up some bots to get real good at StS (Maybe they can help me crack A20).

(Important note: The rng used here is totally different from the actual StS rng, which is way more seeded / correlated. This is partially on purpose (avoid teaching the ais to exploit rng quirks) and partially laziness (I don't want to painstakingly ensure every single rng invocation is done exactly the same way they did.))

TODOs:
- [x] Map
  - [x] Generation
  - [x] Traversal
- [ ] Enemies
  - [ ] Act 1
    - [x] Easy pool
    - [ ] Hard pool
    - [ ] Elites
      - [x] Lagavulin
      - [x] Sentries
      - [ ] Gremlin nob
    - [ ] Bosses
      - [ ] Slime Boss
      - [x] Hexaghost
      - [ ] Guardian
  - [ ] Act 2
  - [ ] Act 3 
- [ ] Potions
  - [ ] Actual potions
  - [x] Potion Rng
  - [ ] Using potions
- [ ] Cards
  - [x] Starter decks
  - [ ] Colorless
  - [ ] Curses
  - [x] Statuses
  - [ ] Ironclad
    - In progress
  - [ ] Other characters
- [ ] Combat
  - [x] Playing cards
  - [x] Damage calculations
  - [x] Buffs and Debuffs
  - [ ] Mid-combat choices
- [ ] Relics
  - Partially implemented
- [ ] Events
- [x] Treasure rooms
- [x] Rewards
- [x] Neow's Blessing