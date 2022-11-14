import pygame, sys

def main():
    w = 200
    h = 300
    win = pygame.display.set_mode((w,h))
    pygame.display.set_caption("Pygame coordinates")

    running = True

    pygame.init()

    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
                pygame.quit()
                sys.exit()

        win.fill((0,0,0))
        pygame.draw.line(win, (255,0,0), (0,0), (0,h), width=2)
        pygame.draw.line(win, (0,255,0), (0,0), (w,0), width=2)

        pygame.draw.line(win, (255,255,255), (50,75), (50,75), width=3)

        pygame.display.flip()

if __name__ == "__main__":
    main()