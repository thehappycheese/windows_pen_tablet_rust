from __future__ import annotations
from typing import Union
from dataclasses import dataclass, field
from html import escape
from math import isnan

@dataclass
class Vec2:
    x: float
    y: float

    def __add__(self, other):
        return Vec2(self.x + other.x, self.y + other.y)

    def __sub__(self, other):
        return Vec2(self.x - other.x, self.y - other.y)

    def __mul__(self, other:Union[int, float, Vec2]) -> Vec2:
        if isinstance(other, Vec2):
            return Vec2(self.x * other.x, self.y * other.y)
        return Vec2(self.x * other, self.y * other)
    
    def __rmul__(self, other:Union[int, float, Vec2]) -> Vec2:
        return self * other

    def __truediv__(self, other:Union[int, float, Vec2]) -> Vec2:
        if isinstance(other, Vec2):
            return Vec2(self.x / other.x, self.y / other.y)
        return Vec2(self.x / other, self.y / other)
    
    def __rtruediv__(self, other:Union[int, float, Vec2]) -> Vec2:
        if isinstance(other, Vec2):
            return Vec2(other.x / self.x, other.y / self.y)
        return Vec2(other / self.x, other / self.y)

    def __neg__(self):
        return Vec2(-self.x, -self.y)
        
    def __str__(self):
        return f"({self.x}, {self.y})"

    def __repr__(self):
        return f"Vec2({self.x}, {self.y})"

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __hash__(self):
        return hash((self.x, self.y))
    
    def transpose(self):
        return Vec2(self.y, self.x)

    def __iter__(self):
        yield self.x
        yield self.y

    def svg(self):
        return f"{self.x} {self.y}"

@dataclass
class Rect:
    top_left: Vec2
    bottom_right: Vec2

    @classmethod
    def from_top_left_size(cls, top_left: Vec2, size: Vec2):
        return cls(top_left, top_left + size)
    
    def size(self):
        return self.bottom_right - self.top_left
    
    def width(self):
        return self.bottom_right.x - self.top_left.x
    
    def height(self):
        return self.bottom_right.y - self.top_left.y
    
    def crop(self, amounts:Vec2):
        return Rect(self.top_left + amounts, self.bottom_right - amounts)

    def crop_top_left(self, amounts:Vec2):
        return Rect(self.top_left + amounts, self.bottom_right)
    
    def crop_bottom_right(self, amounts:Vec2):
        return Rect(self.top_left, self.bottom_right - amounts)

    def crop_left(self, amount:float):
        return Rect(self.top_left + Vec2(amount, 0), self.bottom_right)
    
    def crop_right(self, amount:float):
        return Rect(self.top_left, self.bottom_right - Vec2(amount, 0))
    
    def crop_top(self, amount:float):
        return Rect(self.top_left + Vec2(0, amount), self.bottom_right)
    
    def crop_bottom(self, amount:float):
        return Rect(self.top_left, self.bottom_right - Vec2(0, amount))

    def __str__(self):
        return f"({self.top_left}, {self.bottom_right})"
    
class Attr:
    __attr:dict
    def __init__(self, **kwargs):
        self.__attr = kwargs

    def __str__(self) -> str:
        if len(self.__attr) == 0:
            return ""
        items = [f'{key.replace("_","-")}="{value}"' for key, value in self.__attr.items()]
        total_length = sum(len(item) for item in items)
        join_character = "  " if (total_length < 50 and len(items) < 4) else "\n   "
        result = join_character + join_character.join(items) + join_character
        return result
    
    def __repr__(self) -> str:
        return "< " + " ".join(f'{key}="{value}"' for key, value in self.__attr.items()) + " />"

@dataclass
class Node:
    tag_name:str
    body:list[str|Node] = field(default_factory=list)
    attr:Attr           = field(default_factory=Attr)

    def __str__(self) -> str:
        if not isinstance(self.body, list):
            raise ValueError(f"Invalid body type: {type(self.body)} in tag {self.tag_name}")
        body_strings = []
        for item in self.body:
            if isinstance(item, Node):
                body_strings.append(item.to_string())
            elif isinstance(item, str):
                body_strings.append(escape(item))
            elif isinstance(item, int):
                body_strings.append(str(item))
            elif isinstance(item, float):
                if isnan(item):
                    print(f"Invalid float value: {item} in tag {self.tag_name}")
                body_strings.append(str(item))
            else:
                raise ValueError(f"Invalid body item type {type(item)} {isinstance(item, Node)}: `{item}` in tag {self.tag_name}")
            
        return f"<{self.tag_name}{str(self.attr)}" + (f">{' '.join(body_strings)}</{self.tag_name}>" if len(self.body)>0 else "/>")

    def __repr__(self) -> str:
        return f"<{self.tag_name} />"
    
    def to_string(self) -> str:
        return str(self)