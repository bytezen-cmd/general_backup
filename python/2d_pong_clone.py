import pygame
import sys
import random

pygame.init()

SCREEN_WIDTH = 800
SCREEN_HEIGHT = 600
PADDLE_WIDTH = 15
PADDLE_HEIGHT = 90
BALL_SIZE = 20
PADDLE_SPEED = 5
BALL_SPEED = 7

BLACK = (0, 0, 0)
WHITE = (255, 255, 255)

class Paddle:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.speed = PADDLE_SPEED

    def move_up(self):
        if self.y > 0:
            self.y -= self.speed

    def move_down(self):
        if self.y < SCREEN_HEIGHT - PADDLE_HEIGHT:
            self.y += self.speed

    def draw(self, screen):
        pygame.draw.rect(screen, WHITE, (self.x, self.y, PADDLE_WIDTH, PADDLE_HEIGHT))

    def get_rect(self):
        return pygame.Rect(self.x, self.y, PADDLE_WIDTH, PADDLE_HEIGHT)

class Ball:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.speed_x = BALL_SPEED * random.choice([-1, 1])
        self.speed_y = BALL_SPEED * random.choice([-1, 1])

    def move(self):
        self.x += self.speed_x
        self.y += self.speed_y

        if self.y <= 0 or self.y >= SCREEN_HEIGHT - BALL_SIZE:
            self.speed_y *= -1

    def draw(self, screen):
        pygame.draw.rect(screen, WHITE, (self.x, self.y, BALL_SIZE, BALL_SIZE))

    def get_rect(self):
        return pygame.Rect(self.x, self.y, BALL_SIZE, BALL_SIZE)

    def reset(self):
        self.x = SCREEN_WIDTH // 2 - BALL_SIZE // 2
        self.y = SCREEN_HEIGHT // 2 - BALL_SIZE // 2
        self.speed_x = BALL_SPEED * random.choice([-1, 1])
        self.speed_y = BALL_SPEED * random.choice([-1, 1])

class Game:
    def __init__(self):
        self.screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
        pygame.display.set_caption("Pong")
        self.clock = pygame.time.Clock()

        self.player_paddle = Paddle(30, SCREEN_HEIGHT // 2 - PADDLE_HEIGHT // 2)
        self.ai_paddle = Paddle(SCREEN_WIDTH - 30 - PADDLE_WIDTH, SCREEN_HEIGHT // 2 - PADDLE_HEIGHT // 2)

        self.ball = Ball(SCREEN_WIDTH // 2 - BALL_SIZE // 2, SCREEN_HEIGHT // 2 - BALL_SIZE // 2)

        self.player_score = 0
        self.ai_score = 0
        self.font = pygame.font.Font(None, 74)

    def handle_input(self):
        keys = pygame.key.get_pressed()
        if keys[pygame.K_UP] or keys[pygame.K_w]:
            self.player_paddle.move_up()
        if keys[pygame.K_DOWN] or keys[pygame.K_s]:
            self.player_paddle.move_down()

    def update_ai(self):

        paddle_center = self.ai_paddle.y + PADDLE_HEIGHT // 2
        ball_center = self.ball.y + BALL_SIZE // 2

        if paddle_center < ball_center - 10:
            self.ai_paddle.move_down()
        elif paddle_center > ball_center + 10:
            self.ai_paddle.move_up()

    def check_collision(self):
        ball_rect = self.ball.get_rect()
        player_rect = self.player_paddle.get_rect()
        ai_rect = self.ai_paddle.get_rect()

        if ball_rect.colliderect(player_rect) and self.ball.speed_x < 0:
            self.ball.speed_x *= -1

            hit_pos = (self.ball.y - self.player_paddle.y) / PADDLE_HEIGHT
            self.ball.speed_y = BALL_SPEED * (hit_pos - 0.5) * 2

        if ball_rect.colliderect(ai_rect) and self.ball.speed_x > 0:
            self.ball.speed_x *= -1

            hit_pos = (self.ball.y - self.ai_paddle.y) / PADDLE_HEIGHT
            self.ball.speed_y = BALL_SPEED * (hit_pos - 0.5) * 2

    def check_scoring(self):

        if self.ball.x > SCREEN_WIDTH:
            self.player_score += 1
            self.ball.reset()

        if self.ball.x < -BALL_SIZE:
            self.ai_score += 1
            self.ball.reset()

    def draw_dotted_line(self):

        for i in range(0, SCREEN_HEIGHT, 20):
            if i % 40 == 0:
                pygame.draw.rect(self.screen, WHITE, (SCREEN_WIDTH // 2 - 2, i, 4, 10))

    def draw_scores(self):
        player_text = self.font.render(str(self.player_score), True, WHITE)
        ai_text = self.font.render(str(self.ai_score), True, WHITE)

        self.screen.blit(player_text, (SCREEN_WIDTH // 4, 50))
        self.screen.blit(ai_text, (3 * SCREEN_WIDTH // 4, 50))

    def run(self):

        running = True
        while running:

            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = False
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_ESCAPE:
                        running = False

            self.handle_input()
            self.update_ai()
            self.ball.move()
            self.check_collision()
            self.check_scoring()

            self.screen.fill(BLACK)
            self.draw_dotted_line()
            self.player_paddle.draw(self.screen)
            self.ai_paddle.draw(self.screen)
            self.ball.draw(self.screen)
            self.draw_scores()

            pygame.display.flip()
            self.clock.tick(60)

        pygame.quit()
        sys.exit()

if __name__ == "__main__":
    game = Game()
    game.run()